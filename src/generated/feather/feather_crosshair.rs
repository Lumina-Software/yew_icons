use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_crosshair (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < circle cx = "12" cy = "12" r = "10" /> { "
  " } < line x1 = "22" y1 = "12" x2 = "18" y2 = "12" /> { "
  " } < line x1 = "6" y1 = "12" x2 = "2" y2 = "12" /> { "
  " } < line x1 = "12" y1 = "6" x2 = "12" y2 = "2" /> { "
  " } < line x1 = "12" y1 = "22" x2 = "12" y2 = "18" /> </ svg > } }