we want to plan a frontend for Sanskrit and Pali transliteration using
- [vidyut-lipi](https://github.com/ambuda-org/vidyut/tree/main/vidyut-lipi) which is a transliteration library for Sanskrit and Pali in Rust.
- for frontend we want to use [dioxus](https://github.com/dioxuslabs/dioxus)
- we have created a basic app using dioxus, we use workspace pattern.
- but we will only start with web platfrom only.
- as for basic UI design we like the basic functionalities found in [Lipi Lekhika](https://lipilekhika.in/) which used  another rust transliteration library [lipilekhika](https://github.com/shubhattin/lipilekhika)
    - but we only like the funtionalities of this app, not the UI they used.
    - we more like 
        1. The separation of [Landing page](https://lipilekhika.in/) and [App](https://lipilekhika.in/app/)
            examine the Landing page and App page to understand the separation.
        2. they used Typescript(https://github.com/shubhattin/lipilekhika), our target is to use dioxus.
        3. we prefer the example styling defined in "assets/notes/style.css


[Lipi Lekhika](https://lipilekhika.in/) landing page has a card at center of the page.


```shell
-------------------------
|   नमो नमः -> ನಮೋ ನಮಃ |
-------------------------
```


- this card contains the an example of transliteration for a given text "नमो नमः" is transliterated to all scripts available in vidyut-lipi with time in progress and the result is displayed in the div. 
- for our app we will set the given text as "namatthu buddhānaṁ namatthu bodhiyā. namo vimuttānaṁ namo vimuttiyā."
- we will display the result in the same div with time in progress.


```rust
let text = "namatthu buddhānaṁ namatthu bodhiyā. namo vimuttānaṁ namo vimuttiyā.";
let text = "namo tassa bhagavato arahato sammāsambuddhassa";

``` 

- **Myanmar Unicode Ordering**: Ensures that while `ေ` (ThaVeTho) is rendered on the left, it follows the consonant in the stored byte sequence as per Myanmar Unicode standards.




- **Medial Signs**: Correctly implements medials `ျ` (y), `ြ` (r), `ွ` (w), and `ှ` (h) in clusters when converting from IAST, avoiding redundant stacking/viramas.


Myanmar rendering rules need to fix after perform with `vidyut-lipi`, because `vidyut-lipi` failed to implement some rendering rules for Myanmar. 

- **Tall Vowels**: Automatically uses tall variants (`ါ` and `ေါ`) for consonants `ခ`, `ဂ`, `င`, `ဒ`, `ဓ`, `ပ`, `ဝ`.

Example:

    Pārājikapāḷi -> ပာရာဇိကပာဠိ 
    Corrected: (ပါရာဇိကပါဠိ)

    Buddho Bhagavā -> ဗုဒ္ဓော ဘဂဝာ 
    Corrected:  ဗုဒ္ဓေါ ဘဂဝါ

    kho -> ခော
    Corrected: ခေါ


- **Consonant Clusters**: Properly handles MYANMAR LETTER GREAT SA (ဿ), 
IF Myanmar SA with Virama (္) is used AND followed by same Consonant SA - in such case it is rendered as Myanmar Letter Great SA (ဿ), not stacked SA `သ္သ` 

tas|sa| |sam|mā|sam|bud|dhas|sa

Example:
tassa  sammāsambuddhassa.
... တသ္သ ... သမ္မာသမ္ဗုဒ္ဓသ္သ၊

Corrected: (... တဿ ... သမ္မာသမ္ဗုဒ္ဓဿ၊)


We still FAILED for this case, 

- **Tall Vowels**: Automatically uses tall variants (`ါ` and `ေါ`) for consonants `ခ`, `ဂ`, `င`, `ဒ`, `ဓ`, `ပ`, `ဝ`.
- This case is not as you called as `kho` case, this is not simple as it seems, this apply to all 
    1. LONG VOWEL SIGN A 
    2. O VOWEL SIGN
    this 2 SIGN used with `ခ`, `ဂ`, `င`, `ဒ`, `ဓ`, `ပ`, `ဝ` 
    - current fix only works for `ခ`, `ဂ`, `င`, `ဒ`, `ဓ`, `ပ`, `ဝ` with `LONG VOWEL SIGN A`
    - but 2nd SIGN `O VOWEL SIGN` is not working 

    kho -> ခော
    Corrected: ခေါ

    saddo -> သဒ္ဒော -> Corrected: သဒ္ဒေါ
    Sammāsambuddho -> သမ္မာသမ္ဗုဒ္ဓော -> Corrected: သမ္မာသမ္ဗုဒ္ဓေါ
    
