Доки по клиенту - [https://github.com/locka99/opcua/blob/master/docs/client.md]

OPC UA на данный момент не поддерживает tokio. Поэтому используются синхронные
функции.

Пример полученной структуры из OPC:

```rust
{
    id: 1,
    client_handle: 1000,
    item_to_monitor: ReadValueId {
        node_id: NodeId {
            namespace: 4,
            identifier: Numeric(2)
        },
        attribute_id: 13,
        index_range: UAString { value: None },
        data_encoding: QualifiedName {
            namespace_index: 0,
            name: UAString { value: None }
        }
    },
    queue_size: 1,
    discard_oldest: true,
    monitoring_mode: Reporting,
    sampling_interval: 1000.0,
    last_value: DataValue {
        value: Some(Int16(0)),
        status: None,
        source_timestamp: Some(DateTime {
            date_time: 2023-08-15T17:24:40.453701900Z
        }),
        source_picoseconds: None,
        server_timestamp: Some(DateTime {
            date_time: 2023-08-15T17:24:40.453701900Z
        }),
        server_picoseconds: None
    },
    values: [
        DataValue {
            value: Some(Int16(0)),
            status: None,
            source_timestamp: Some(DateTime {
                date_time: 2023-08-15T17:24:40.453701900Z
            }),
            source_picoseconds: None,
            server_timestamp: Some(DateTime {
                date_time: 2023-08-15T17:24:40.453701900Z
            }),
            server_picoseconds: None }
    ],
    triggered_items: {}
}
```
