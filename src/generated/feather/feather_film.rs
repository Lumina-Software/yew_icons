use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_film (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < rect x = "2" y = "2" width = "20" height = "20" rx = "2.18" ry = "2.18" /> { "
  " } < line x1 = "7" y1 = "2" x2 = "7" y2 = "22" /> { "
  " } < line x1 = "17" y1 = "2" x2 = "17" y2 = "22" /> { "
  " } < line x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> { "
  " } < line x1 = "2" y1 = "7" x2 = "7" y2 = "7" /> { "
  " } < line x1 = "2" y1 = "17" x2 = "7" y2 = "17" /> { "
  " } < line x1 = "17" y1 = "17" x2 = "22" y2 = "17" /> { "
  " } < line x1 = "17" y1 = "7" x2 = "22" y2 = "7" /> </ svg > } }