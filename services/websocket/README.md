Рассылка новых сообщений из Redis по Websocket

Идеи для улучшения:

- TODO посылать не сразу, а собирать в буфер и посылать раз в 500мс, например.
  У сообщений одного типа посылать только самые новые. Уменьшит нагрузку на сеть,
  на клиенте данные (возможно) будут обновляться не вразнобой, а синхронно.

- TODO Получать данные не через api, а через канал websocket.

- TODO Убрать unwrap

- TODO tasks::listen_port не работает с дженериком вместо Messages
