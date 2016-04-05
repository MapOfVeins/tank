## Tank

Tank is an html templating system written in Rust, with syntax similar to Rust itself.

    
A simple example:

    div (id:my-div class:styled-div) ->
        p () -> I am driving a tank
	
becomes:

    <div id="my-div" class="styled-div">
        <p>
          I am driving a tank
        </p>
    </div>
