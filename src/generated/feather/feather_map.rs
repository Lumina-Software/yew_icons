use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_map (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polygon points = "1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6" /> { "
  " } < line x1 = "8" y1 = "2" x2 = "8" y2 = "18" /> { "
  " } < line x1 = "16" y1 = "6" x2 = "16" y2 = "22" /> </ svg > } }