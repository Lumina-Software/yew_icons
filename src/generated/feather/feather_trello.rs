use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_trello (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> { "
  " } < rect x = "7" y = "7" width = "3" height = "9" /> { "
  " } < rect x = "14" y = "7" width = "3" height = "5" /> </ svg > } }