use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_igloo (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 576 512" >< path d = "M320 160H48.5C100.2 82.82 188.1 32 288 32C298.8 32 309.5 32.6 320 33.76V160zM352 39.14C424.9 55.67 487.2 99.82 527.5 160H352V39.14zM96 192V320H0C0 274 10.77 230.6 29.94 192H96zM192 320H128V192H448V320H384V352H576V432C576 458.5 554.5 480 528 480H352V352C352 316.7 323.3 288 288 288C252.7 288 224 316.7 224 352V480H48C21.49 480 0 458.5 0 432V352H192V320zM480 192H546.1C565.2 230.6 576 274 576 320H480V192z" /></ svg > } }