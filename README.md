## Tank

Tank is an html templating system written in Rust, with syntax similar to Rust itself.

    
A simple example:

    div (id: my-div class: styled-div) ->
        p () -> I am driving a tank
	
becomes:

    <div id="my-div" class="styled-div">
        <p>
          I am driving a tank
        </p>
    </div>
    
### Including other templates

You can reference other html files or tank files, and have their contents inserted in place. For example, with a header
template to use on multiple pages, you can insert it into your index page like so:

header.tank

    section(id: header) -> Here is the header

index.tank

    &header
    div(id: welcome-banner) -> Welcome!
    
index.html

    <section id="header"> Here is the header </section>
    <div id="welcome-banner"> Welcome! </div>
