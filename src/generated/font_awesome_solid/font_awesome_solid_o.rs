use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_o (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 448 512" >< path d = "M224 32.01c-123.5 0-224 100.5-224 224s100.5 224 224 224s224-100.5 224-224S347.5 32.01 224 32.01zM224 416c-88.22 0-160-71.78-160-160s71.78-159.1 160-159.1s160 71.78 160 159.1S312.2 416 224 416z" /></ svg > } }