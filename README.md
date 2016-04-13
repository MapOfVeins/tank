## Tank

Tank is an html templating system written in Rust, with syntax similar to Rust itself.

    
A simple example:

```tank
div (id: my-div class: styled-div) ->
    p () -> I am driving a tank
```

becomes:

```html
<div id="my-div" class="styled-div">
  <p>
    I am driving a tank
  </p>
</div>
```
    
### Including other templates

You can reference other html files or tank files, and have their contents inserted in place. For example, with a header
template to use on multiple pages, you can insert it into your index page like so:

header.tank

```tank
section(id: header) -> Here is the header
```

index.tank

```tank
&header
div(id: welcome-banner) -> Welcome!
```

These files together will compile to the following:

```html
<section id="header"> Here is the header </section>
<div id="welcome-banner"> Welcome! </div>
```

### Declaring variables

Variables can be declared inside tank files, and the value can be accessed using the '%' operator:

```tank
let myVar: int = 10
div() -> %myVar
p() -> myVar
```

becomes:

```html
<div>
  10
</div>
<p>
  myVar
</p>
```



