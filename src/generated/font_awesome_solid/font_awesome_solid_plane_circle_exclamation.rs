use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_plane_circle_exclamation (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 640 512" >< path d = "M320 93.68V178.3L397.1 222.4C350.6 254 320 307.4 320 368C320 422.2 344.5 470.7 383.1 502.1C381 508.3 375.9 512 369.1 512C368.7 512 367.4 511.8 366.1 511.5L256 480L145.9 511.5C144.6 511.8 143.3 512 142 512C134.3 512 128 505.7 128 497.1V456C128 450.1 130.4 446.2 134.4 443.2L192 400V329.1L20.4 378.2C10.17 381.1 0 373.4 0 362.8V297.3C0 291.5 3.076 286.2 8.062 283.4L192 178.3V93.68C192 59.53 221 0 256 0C292 0 320 59.53 320 93.68H320zM352 368C352 288.5 416.5 224 496 224C575.5 224 640 288.5 640 368C640 447.5 575.5 512 496 512C416.5 512 352 447.5 352 368zM496 464C509.3 464 520 453.3 520 440C520 426.7 509.3 416 496 416C482.7 416 472 426.7 472 440C472 453.3 482.7 464 496 464zM479.1 288V368C479.1 376.8 487.2 384 495.1 384C504.8 384 511.1 376.8 511.1 368V288C511.1 279.2 504.8 272 495.1 272C487.2 272 479.1 279.2 479.1 288z" /></ svg > } }