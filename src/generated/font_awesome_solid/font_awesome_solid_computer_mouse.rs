use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_computer_mouse (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 384 512" >< path d = "M0 352c0 88.38 71.63 160 160 160h64c88.38 0 160-71.63 160-160V224H0V352zM176 0H160C71.63 0 0 71.62 0 160v32h176V0zM224 0h-16v192H384V160C384 71.62 312.4 0 224 0z" /></ svg > } }