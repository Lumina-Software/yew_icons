use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_whiskey_glass (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 512 512" >< path d = "M479.1 32H32.04C12.55 32-2.324 49.25 .3008 68.51L56.29 425.1C60.79 456.6 87.78 480 119.8 480h272.9c31.74 0 58.86-23.38 63.36-54.89l55.61-356.6C514.3 49.25 499.5 32 479.1 32zM422.7 224H89.49L69.39 96h373.2L422.7 224z" /></ svg > } }