
pub mod configuration; 
pub mod routes;
pub mod startup;

/*
تمام ✨ خليني أشرحلك النقطة دي تحديدًا:

### 🎯 ما معنى إن `lib.rs` هو "البوابة الرئيسية" ويعمل **re-export**؟

* في Rust، كل **crate** (المشروع نفسه) بيكون ليه ملف **رئيسي** اسمه `lib.rs` (لو مكتبة) أو `main.rs` (لو تطبيق).
* `lib.rs` هنا هو المكان اللي **يجمع كل الموديولات الفرعية** (sub-modules) ويعيد تصديرها (re-export) علشان أي ملف تاني في المشروع يقدر يستخدمها بسهولة.

---

### 🗂️ مثال عملي

#### لو عندك ملفات بالشكل ده:

```
src/
  lib.rs
  routes/
    mod.rs
    health_check.rs
    subscriptions.rs
```

#### محتوى `routes/mod.rs`:

```rust
mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;
```

هنا `pub use` معناها:

> "أنا هاخد كل ما هو عام (pub) من `health_check` و `subscriptions` وأخليه متاح من خلال `routes`".

---

#### محتوى `lib.rs`:

```rust
pub mod configuration;
pub mod routes;
pub mod startup;
```

هنا بيقول:

* أي حد عايز يستخدم الـ `routes` (بما فيها `health_check` و `subscriptions`) مش لازم يروح يدور جوا الملفات الداخلية.
* يكفي يكتب:

  ```rust
  use zero2prod::routes::*;
  ```

بدل ما يكتب:

```rust
use zero2prod::routes::health_check::some_function;
use zero2prod::routes::subscriptions::another_function;
```

---

### 💡 الفائدة

* `lib.rs` يعمل زي **واجهة API داخلية** للكود بتاعك.
* بدل ما تجبر كل ملف جديد يعرف تفاصيل المجلدات الداخلية (فين `health_check.rs` أو `subscriptions.rs`)،
  بتقدمله "باب رئيسي مرتب" يدخل منه على طول.

---

تحب أديك analogy مبسطة كأنك عندك **شركة** و`lib.rs` هو **الريسيبشن** اللي بيجمع الخدمات كلها ويوجهك بدل ما تدخل تدور بنفسك؟

*/