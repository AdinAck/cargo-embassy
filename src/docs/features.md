## embassy-executor

**executor-interrupt**

Enables the ability to drive an executor with an interrupt.

> Example: https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/multiprio.rs
> Learn more: https://embassy.dev/book/dev/runtime.html#_interrupts

**task-arena-size-?** - *Default:* `task-arena-size-4096`

Configures the task arena size available to the executor.

> Learn more: https://embassy.dev/book/dev/faq.html#_how_do_i_set_up_the_task_arenas_on_stable

## embassy-time

**tick-hz-?**

Configures the tick rate of the active time driver.

> This **must** be specified as the default of `1MHz` is not operational.
> A typical value is `tick-hz-32_768` but this should be set appropriately for the application.
> Learn more: https://github.com/embassy-rs/embassy/tree/main/embassy-time#tick-rate

**defmt-timestamp-uptime**

Enables the reporting of the uptime (seconds) in RTT messages.

## embassy-stm32

**exti**

Enables the use of the EXTI peripheral.

> Example: https://github.com/embassy-rs/embassy/blob/main/examples/stm32g4/src/bin/button_exti.rs

**time-driver-?**

Configures the time driver to be provided to `embassy-time`.

> An exact timer to be used can be specified with `time-driver-timX` where `X` is the timer number.
> `embassy-stm32` can pick a time driver itself with `time-driver-any`.
> Learn more: https://github.com/embassy-rs/embassy/tree/18da9a2b66f21a1d1b5cd07c8567b700be8c7b09/embassy-stm32#embassy-time-time-driver

**time**

Enables additional time related functionality.

> In general, enables timouts for IO transactions.

## embassy-nrf

**time-driver-rtc1**

Configures the time driver to be provided to `embassy-time`.

> Only one timer (`RTC1`) is available as a time driver.
> Learn more: https://github.com/embassy-rs/embassy/tree/18da9a2b66f21a1d1b5cd07c8567b700be8c7b09/embassy-nrf#time-driver

**time**

Enables additional time related functionality.

> In general, enables timouts for IO transactions.
