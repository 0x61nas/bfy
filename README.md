<p align="center"> 

<img alt="brainfuc*k interpreter" src="./assets/cover.png" width="100%" />

</p>

# ⁭مترجم brainfu* k: مترجم بسيط من Brainfuc * K و REPL مكتوب في Rust 🦀

## سمات
- قم بتشغيل كود brainfu * k من ملف 💫
- قم يسمح Brainfu * k الكود المباشر من REPL 🚀
- دعم أحرف UTF-8 بما في ذلك الرموز التعبيرية 😍 (اختياري)
- يمكنك التحكم في حجم المصفوفة 📏

## تثبيت

- تثبيت crates.io:
    ```shell
    cargo install bfy
    ```
- من aur: 
  ```shell
  yay -S bfy
  ```

## الخيارات والحجج
<details>
   <summary>جدول الخيارات</summary>
   <table>
      <thead>
         <tr>
            <th>اختيار</th>
            <th>وصف</th>
            <th>إفتراضي</th>
         </tr>
      </thead>
      <tbody>
         <tr>
            <td><code>-h</code>, <code>--help</code></td>
            <td>طباعة معلومات المساعدة</td>
            <td></td>
         </tr>
         <tr>
            <td><code>-V</code>, <code>--version</code></td>
            <td>يطبع معلومات الإصدار</td>
            <td></td>
         </tr>
         <tr>
            <td><code>-f</code>, <code>--features</code></td>
            <td>
               الميزات الإضافية للتمكين<br/>
               القيم الممكنة:
               <ul>
                  <li>no-reverse-value:<br/>
                     إذا كانت القيمة تريد إنقاص القيمة وكانت القيمة 0 ، فلا تعين القيمة على 255 ، وإلا قلل القيمة. <br/>
                      إذا كانت القيمة تريد زيادة القيمة وكانت القيمة 255 ، فلا تقم بتعيين القيمة على 0 ، وإلا قم بزيادة القيمة. الاسم المستعار هو:
                      "nrv"
                  </li>
                  <li>reverse-pointer:<br/>
                    إذا كان المؤشر في نهاية المصفوفة ، فاضبط المؤشر على 0 ، وإلا قم بزيادة المؤشر. <br/>
                      إذا كان المؤشر في بداية المصفوفة ، فاضبط المؤشر على نهاية المصفوفة ، وإلا قلل المؤشر. الاسم المستعار هو: `rp`
                  </li>
                  <li>allow-utf8:<br/>
                     اسمح باستخدام أحرف utf8 (32 بت) ، وإلا فلن يُسمح إلا باستخدام أحرف 8 بت. <br/>
                      استخدم هذه الميزة بحذر لأنها تزيد حجم الخلية من 8 بت إلى 32 بت. <br/>
                      كما يسمح لك باستخدام الرموز التعبيرية في كود مخادعك: D ، هذا إذا استطعت
                      حافظ على عقلك حتى تتمكن من الوصول إلى قيمتها الرقمية :). <br/>
                      يمكن لـ "u32" في الصدأ تخزين القيم من 0 إلى 4294967295 فقط ، لكننا
                      يمكن فقط استخدام 0 إلى 1114111 (0x10FFFF) في الوقت الحالي. الاسم المستعار هو: `utf8`
                  </li>
               </ul>
            </td>
            <td>غير متوفر</td>
         </tr>
         <tr>
            <td><code>-a</code>, <code>--array-size</code></td>
            <td>حجم المصفوفة</td>
            <td>30000</td>
         </tr>
         <tr>
            <td><code>-w</code>, <code>--without-tiles</code></td>
            <td>لا تقم بطباعة المربعات (مثل رمز الخروج واسم الملف وما إلى ذلك)</td>
            <td></td>
         </tr>
      </tbody>
   </table>
</details>


<details>
<summary>مساعدة خيار الإخراج</summary>

```shell
bfy --help
```
```text
Brainfu*k interpreter and REPL written in Rust

Usage: bfy [OPTIONS] [SOURCE]

Arguments:
  [SOURCE]
          The brainfuck source code file to run (if not will be entered in REPL mode)

Options:
  -f, --features <FEATURES>
          Possible values:
          - no-reverse-value:
            If the value is you want decrement the value and the value is 0, don't set the value to 255, otherwise decrement the value. If the
            value is you want increment the value and the value is 255, don't set the value to 0, otherwise increment the value. The alias are:
            `nrv`
          - reverse-pointer:
            If the pointer at the end of the array, set the pointer to 0, otherwise increment the pointer. If the pointer at the beginning of the
            array, set the pointer to the end of the array, otherwise decrement the pointer. The alias are: `rp`
          - allow-utf8:
            Allow the use of utf8 characters (32 bit), otherwise only 8 bit characters are allowed. Use this feature with caution because it
            increases the cell size from 8 bits to 32 bits. It also allow you to use the emoji in your brainfuck code :D, This is if you can
            preserve your mind so that you can access their digital value :). The `u32` in rust can only store values from 0 to 4294967295, but we
            can only use 0 to 1114111 (0x10FFFF) for now. The alias are: `utf8`

  -a, --array-size <ARRAY_SIZE>
          The brainfuck array size
          
          [default: 30000]

  -w, --without-tiles
          Dont print the tiles (e.g. exit code, file name, etc)

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

</details>

### أمثلة

```bash
bfy test_code/hello_world.bf
```
```text
Hello world!
Successfully ran brainfuck source code from file: test_code/hello_world.bf
Exiting with code: 0
```

```bash
bfy -w test_code/hello_world.bf
```
```text
Hello world!
```

```bash
bfy test_code/print_hi_yooo.bf
```
```text
Hi yoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo!Successfully ran brainfuck source code from file: test_code/print_hi_yooo.bf
Exiting with code: 0
```

```bash
bfy -w test_code/print_hi_yooo.bf
```
```text
Hi yoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo!
```

```bash
bfy test_code/like_cat.bf
```
![output](./screenshots/like_cat_output.png)
> ملاحظة: الإخراج ليس هو نفسه الإصدار الحالي للمترجم الفوري ، ولكنه مماثل لإخراج المترجم الفوري عندما أكتب الكود.

## REPL
```bash
bfy # REPL mode
```
![print @ and A in the repl](./screenshots/repl_print_at_and_A_0.1.0.png)

## لكى يفعل
- [ ] أضف المزيد من الاختبارات
- [ ] أضف المزيد من الأمثلة
- [ ] قم بإنشاء منسق brainfu * k
- [ ] أضف تمييز بناء الجملة في REPL
- [ ] أضف الإكمال التلقائي في REPL
- [ ] دعم ميزة حجم الصفيف الديناميكي

## مصادر
- [مخنث فى ويكيبيديا](https://en.wikipedia.org/wiki/Brainfuck)
- [البرمجة في Brainfuck](http://cydathria.com/bf/brainfuck.html)
- [Brainfuck: لغة برمجة Turing-Complete ذات ثمانية تعليمات](http://www.muppetlabs.com/~breadbox/bf)
- [Brainfuc * k- متخيل](https://github.com/usaikiran/brainfuck-visualizer)
- [أساسيات BrainFuck](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
- [لغة برمجة أنيقة: Brainfuck](https://www.neperos.com/article/raqehg6b24ceadba)
