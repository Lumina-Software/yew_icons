use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_download_cloud (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polyline points = "8 17 12 21 16 17" /> { "
  " } < line x1 = "12" y1 = "12" x2 = "12" y2 = "21" /> { "
  " } < path d = "M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29" /> </ svg > } }