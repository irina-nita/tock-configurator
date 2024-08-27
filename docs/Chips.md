Supported chips
==============

The `chips` directory keeps the supported chips implementations as crates. Each supported peripheral
must implement the corresponding traits from the `parse` crate.

### Important

This approach is not yet considered scalable. The approach this should be moving towards is to have
chip configuration `JSON` files, similar to:

```json
// nrf52833.code.json 
{
	"chip": {
		"init_expr": null,
		"dependenicies": null,
		"after_init": null,
		"before_init": null,
		"ident": "auto"
	}
}
```

```json
// nrf52833.peripherals.json 
{
    "peripherals": [
        {
            "uart": [
                "uart0",
                "uart1"
            ]
        },
		{
			"timer": [
				"rtc"
			]
		}
    ],
    "systick": null
}
```

