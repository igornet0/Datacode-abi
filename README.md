# Datacode abi

Минимальный стабильный **C-совместимый контракт** между виртуальной машиной DataCode и нативными модулями (плагинами). Крейт задаёт только типы, версию ABI и границу VM ↔ модуль — **единый источник правды** для FFI.

## Назначение

- Описание значений на границе вызова (`AbiValue`).
- Версия ABI и проверка совместимости.
- Дескрипторы модуля и таблицы экспорта (функции, классы, глобальные привязки).
- Контекст VM для регистрации нативов и обратных вызовов (`VmContext`).
- Коды ошибок для проброса в VM (`DatacodeError`).

Логику моста VM ↔ внутренние значения языка реализует хост (например, `abi_bridge` в VM/SDK); этот крейт её не содержит.

## Зависимость в Rust

```toml
datacode_abi = { path = "datacode_sdk/datacode_abi" }
```

Для удобной сборки модулей на Rust обычно используют **`datacode_sdk`** (`define_module!`, `define_module_descriptor!`, `define_module_entry!`), который зависит от `datacode_abi`.

## Версия ABI

Структура **`AbiVersion`** — `{ major, minor }` как два `u16`, пригодна для C/FFI.

Текущая версия контракта: **`DATACODE_ABI_VERSION`** (см. `src/version.rs`).

**Совместимость** — функция **`abi_compatible(module, vm)`**: одинаковый `major` и `module.minor <= vm.minor`.

Правило эволюции:

- увеличивать **minor** при добавочных изменениях макета, которые старые загрузчики ещё переносят;
- увеличивать **major** при несовместимых изменениях.

В комментариях к `DATACODE_ABI_VERSION` зафиксированы вехи minor (например, корневой `AbiModuleDescriptor`, таблицы классов/глобалов, вариант `PluginOpaque` в значениях).

## Точки входа нативного модуля

| Символ | Тип | Назначение |
|--------|-----|------------|
| **`datacode_module_entry`** (`DATACODE_MODULE_ENTRY_SYMBOL`) | `DatacodeModuleEntryFn` → `*const AbiModuleDescriptor` | Предпочтительный путь: один корневой дескриптор, без колбэка `register`. |
| **`datacode_module`** (`DATACODE_MODULE_SYMBOL`) | `DatacodeModuleFn` → `*const DatacodeModule` | Переходный/legacy: обёртка с `DatacodeModule` или legacy-структурой. |

VM проверяет **`abi_version`** в дескрипторе до чтения таблиц.

## Режимы загрузки (1.0 vs 1.1+)

После проверки ABI VM смотрит на **`minor`**:

- **`minor == 0`** — layout **`DatacodeModuleLegacy`**: три поля (`abi_version`, `name`, `register`). Экспорты собираются через вызов `register` и «подменный» `VmContext` (`register_native`).
- **`minor >= 1`** — layout **`DatacodeModule`**:
  - если **`export_table` не null** — экспорты читаются **только** из **`AbiExportTable`** / **`AbiExport`**, **`register` не вызывается**;
  - если `export_table` null и **`register` задан** — используется тот же путь регистрации, что и в legacy;
  - если оба варианта отсутствуют — загрузка не удаётся.

Корневой **`AbiModuleDescriptor`** (через `datacode_module_entry`) описывает плоские функции, классы с методами и модульные глобалы (геттеры).

## Значения и вызов нативов

**`AbiValue`** (`Value`) — `#[repr(C)]` перечисление: целые, float, bool, строка (UTF-8, null-terminated, не владеет указателем), null, массив (`ptr` + длина), объект (`NativeHandle`), плюс **`PluginOpaque { tag, id }`** для непрозрачных объектов плагина.

Сигнатура натива:

```text
NativeAbiFn = extern "C" fn(*mut VmContext, *const AbiValue, argc) -> AbiValue
```

Указатели и строки от VM считаются валидными на время одного вызова натива.

### `PluginOpaque` и `isinstance` на стороне VM

Хост сопоставляет тип значения с именем, которое возвращает хук **`opaque_type_name`** из `AbiPluginHooksDescriptor` в модуле `module` (по тегу `PluginOpaque`).

Чтобы выражение вида `isinstance(x, dataset)` работало без хардкода имён модулей в компиляторе:

- если второй аргумент — **экспорт натива** с именем `foo` (плоский `NativeFunction`), он сопоставляется с `opaque_type_name(x)` при совпадении имён без учёта регистра;
- если второй аргумент — **объект пространства имён** вложенного модуля (ключ **`__plugin_namespace`** = `"foo"`, как у `ml.dataset` после `nest_dotted` экспортов), сравнение такое же.

**Конвенция для авторов плагинов:** строка, которую возвращает `opaque_type_name` для данного тега, должна совпадать с **именем экспорта** конструктора/типа (`dataset`, `tensor`, …) или с сегментом namespace для вложенных API (`dataset` для `dataset.*`).

**Ленивые `iterable` в языке** (`map` / `filter` и т.д.) в `AbiValue` **не передаются**: перед вызовом нативного модуля VM разворачивает их в обычный **`Array`** (один проход итератора). На границе FFI плагин по-прежнему видит только описанные выше варианты `AbiValue`.

## VmContext

Передаётся в колбэк **`register`**. Модуль использует только поля:

- **`alloc`** — выделение памяти через аллокатор VM;
- **`throw_error`** — сообщение об ошибке с кодом **`DatacodeError`** (интеграция с try/catch на стороне VM);
- **`register_native`** — регистрация функции по имени и **`NativeAbiFn`**.

## Ошибки

**`DatacodeError`** — `#[repr(C)]` enum: `Ok`, `TypeError`, `RuntimeError`, `Panic`.

## Структура крейта

| Модуль | Содержимое |
|--------|------------|
| `version` | `AbiVersion`, `DATACODE_ABI_VERSION`, `abi_compatible` |
| `value` | `AbiValue`, `NativeHandle` |
| `error` | `DatacodeError` |
| `vm_context` | `VmContext`, `NativeAbiFn` |
| `module` | `AbiExport`, `AbiExportTable`, `AbiClassDescriptor`, `AbiGlobalDescriptor`, `AbiModuleDescriptor`, `DatacodeModule`, `DatacodeModuleLegacy`, символы входа |

## Лицензия

MIT (см. `LICENSE` в корне пакета).
