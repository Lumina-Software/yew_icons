use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_instagram (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < rect x = "2" y = "2" width = "20" height = "20" rx = "5" ry = "5" /> { "
  " } < path d = "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" /> { "
  " } < line x1 = "17.5" y1 = "6.5" x2 = "17.51" y2 = "6.5" /> </ svg > } }