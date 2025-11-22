# 🚀 Deployment Guide - URL Shortener

## Повний гайд по деплою на 100% безкоштовний хостинг

### Крок 1: Підготовка Cloudflare

#### 1.1 Створити акаунт
1. Зареєструватися на [cloudflare.com](https://cloudflare.com)
2. Перейти в розділ Workers & Pages

#### 1.2 Створити KV Namespace
```bash
wrangler login
wrangler kv:namespace create "URLS"
```

Отримаєте щось типу:
```
{ binding = "URLS", id = "abc123..." }
```

Скопіюйте `id` і вставте в `wrangler.toml`:
```toml
[[kv_namespaces]]
binding = "URLS"
id = "abc123..."  # ← ваш ID
```

#### 1.3 Створити D1 Database
```bash
wrangler d1 create url_shortener_db
```

Отримаєте:
```
database_id = "xyz789..."
```

Оновіть `wrangler.toml`:
```toml
[[d1_databases]]
binding = "DB"
database_name = "url_shortener_db"
database_id = "xyz789..."  # ← ваш ID
```

#### 1.4 Ініціалізувати базу даних
```bash
wrangler d1 execute url_shortener_db --file=./schema.sql
```

### Крок 2: Деплой Cloudflare Worker (Backend)

#### 2.1 Встановити залежності
```bash
# Встановити Rust (якщо ще не встановлено)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Встановити worker-build
cargo install worker-build

# Встановити wrangler
npm install -g wrangler
```

#### 2.2 Зібрати проект
```bash
worker-build --release
```

#### 2.3 Деплой
```bash
wrangler deploy
```

Отримаєте URL типу:
```
https://url-shortener.YOUR_SUBDOMAIN.workers.dev
```

**Збережіть цей URL!** Він потрібен для фронтенду.

### Крок 3: Деплой Frontend (GitHub Pages)

#### 3.1 Створити GitHub репозиторій
1. Перейти на [github.com](https://github.com)
2. Створити новий репозиторій, наприклад `shortlink`
3. **НЕ** додавати README, .gitignore, або ліцензію

#### 1. Оновити app.js
Відкрити `static/app.js` і змінити:
```javascript
const API_URL = 'https://url-shortener.YOUR_SUBDOMAIN.workers.dev';
```

#### 2. Deploy Frontend (GitHub Pages)

Ми використовуємо папку `docs/` для GitHub Pages (це спрощує налаштування).

1. **Ініціалізуйте Git (якщо ще ні):**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   ```

2. **Створіть репозиторій на GitHub:**
   - Перейдіть на [GitHub.com](https://github.com/new)
   - Створіть новий публічний репозиторій (наприклад `shortlink`)

3. **Запуште код:**
   ```bash
   git remote add origin https://github.com/YOUR_USERNAME/shortlink.git
   git branch -M main
   git push -u origin main
   ```

4. **Увімкніть GitHub Pages:**
   - Перейдіть в **Settings** -> **Pages**
   - Source: **Deploy from a branch**
   - Branch: **main**
   - Folder: **/docs** (важливо!)
   - Натисніть **Save**

Ваш сайт буде доступний за адресою: `https://YOUR_USERNAME.github.io/shortlink/`

#### 3. Оновити BASE_URL в Worker
Відкрити `wrangler.toml` і змінити:
```toml
[vars]
BASE_URL = "https://YOUR_USERNAME.github.io/shortlink"
```

Передеплоїти Worker:
```bash
wrangler deploy
```

### Крок 4: Тестування

#### 4.1 Тест фронтенду
1. Відкрити `https://YOUR_USERNAME.github.io/shortlink/`
2. Ввести URL: `https://google.com`
3. Натиснути "Скоротити"
4. Повинно створитися коротке посилання

#### 4.2 Тест редіректу
1. Скопіювати коротке посилання
2. Відкрити в новій вкладці
3. Повинно редіректнути на оригінальний URL

#### 4.3 Тест API
```bash
# Скоротити URL
curl -X POST https://url-shortener.YOUR_SUBDOMAIN.workers.dev/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# Повинно повернути:
{
  "short_url": "https://YOUR_USERNAME.github.io/shortlink/abc123",
  "short_code": "abc123",
  "original_url": "https://example.com"
}
```

### Крок 5: Налаштування Stripe (Опціонально)

#### 5.1 Створити Stripe акаунт
1. Зареєструватися на [stripe.com](https://stripe.com)
2. Отримати API ключі (Developers → API keys)

#### 5.2 Додати секрети
```bash
wrangler secret put STRIPE_SECRET_KEY
# Вставити ваш sk_test_... або sk_live_...

wrangler secret put STRIPE_WEBHOOK_SECRET
# Вставити whsec_...
```

#### 5.3 Налаштувати webhook
1. Stripe Dashboard → Developers → Webhooks
2. Add endpoint: `https://url-shortener.YOUR_SUBDOMAIN.workers.dev/api/webhooks/stripe`
3. Events: `customer.subscription.created`, `customer.subscription.updated`, `customer.subscription.deleted`

### Крок 6: Власний домен (Опціонально)

#### 6.1 Купити домен
Рекомендації:
- Namecheap: `s.link`, `short.dev` (~$10-50/рік)
- Cloudflare Registrar (найдешевше)

#### 6.2 Підключити до Cloudflare
1. Cloudflare Dashboard → Add site
2. Додати ваш домен
3. Змінити nameservers у реєстратора

#### 6.3 Налаштувати Worker
1. Workers & Pages → url-shortener → Settings → Triggers
2. Add Custom Domain
3. Ввести ваш домен (наприклад, `s.link`)

#### 6.4 Оновити BASE_URL
```toml
[vars]
BASE_URL = "https://s.link"  # ← ваш домен
```

```bash
wrangler deploy
```

## 🎉 Готово!

Ваш URL Shortener працює на:
- ✅ Backend: Cloudflare Workers (безкоштовно до 100k запитів/день)
- ✅ Frontend: GitHub Pages (безкоштовно назавжди)
- ✅ Database: Cloudflare D1 (безкоштовно до 5GB)
- ✅ Storage: Cloudflare KV (безкоштовно до 1GB)

**Вартість: $0/міс** до перших клієнтів! 💰

## 📊 Моніторинг

### Cloudflare Analytics
1. Workers & Pages → url-shortener → Analytics
2. Переглядати запити, помилки, затримки

### GitHub Pages
1. Repository → Insights → Traffic
2. Переглядати відвідувачів

## 🔄 Оновлення

### Оновити Worker
```bash
# Внести зміни в код
# ...

# Передеплоїти
wrangler deploy
```

### Оновити Frontend
```bash
cd static

# Внести зміни
# ...

# Commit і push
git add .
git commit -m "Update frontend"
git push
```

GitHub Pages автоматично оновиться за 1-2 хвилини.

## 🐛 Troubleshooting

### Worker не працює
```bash
# Перевірити логи
wrangler tail

# Перевірити конфігурацію
wrangler whoami
```

### GitHub Pages не оновлюється
1. Settings → Pages → перевірити статус
2. Actions → перевірити build logs

### База даних порожня
```bash
# Переініціалізувати
wrangler d1 execute url_shortener_db --file=./schema.sql
```

## 💡 Наступні кроки

1. ✅ Додати Google Analytics
2. ✅ Налаштувати Stripe для монетизації
3. ✅ Створити dashboard для користувачів
4. ✅ Додати автентифікацію
5. ✅ Запустити рекламу (Google Ads, Facebook)

---

**Потрібна допомога?** Створіть issue на GitHub!
