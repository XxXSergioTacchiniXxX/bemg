# Bemg
Generation of scss BEM classes from html-like files.

The simplest CLI utility that generates SCSS markup according to BEM notation. The utility accepts only one parameter - the path to the desired file. As a result, SCSS with blocks, elements and BEM modifiers is returned to the standard output.
It works similarly to the functionality of [BEM Helper](https://marketplace.visualstudio.com/items?itemName=Box-Of-Hats.bemhelper).

Example:

```html
<div class="sub-form-1">
  <div class="sub-form-1__body">
    <div class="sub-form-1__info">
      <h2>Title</h2>
      <span>Text</span>
    </div>

    <form action="#" method="post" class="sub-form-1__form">
      <div class="sub-form-1__form-input">
        <input class="input-text-1" />
      </div>

      <div class="sub-form-1__form-submit">
        <input class="button-1 button-1--white" />
      </div>
    </form>
  </div>
</div>
```
The following SCSS is obtained from this HTML:

```scss
.sub-form-1 {
   &__info {}
   &__form-submit {}
   &__form {}
   &__form-input {}
   &__body {}
}
.button-1 {
   &--white {}
}
.input-text-1 {
}
```
This utility handles the single-file components of vue js and svelte well. JSX support is planned in the future.
