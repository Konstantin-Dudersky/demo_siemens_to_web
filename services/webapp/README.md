Запуск для отладки:

```
trunk serve --open
```

Сборка в релиз:

```bash
npx tailwindcss -o ./style/output.css --minify | trunk build --release
```
