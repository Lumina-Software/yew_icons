use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_minus_square (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> { "
  " } < line x1 = "8" y1 = "12" x2 = "16" y2 = "12" /> </ svg > } }