use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ne (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-ne" viewBox = "0 0 512 512" > < path fill = "#0db02b" d = "M0 0h512v512H0z" /> { "
  " } < path fill = "#fff" d = "M0 0h512v341.3H0z" /> { "
  " } < path fill = "#e05206" d = "M0 0h512v170.7H0z" /> { "
  " } < circle cx = "256" cy = "256" r = "72.5" fill = "#e05206" /> </ svg > } }