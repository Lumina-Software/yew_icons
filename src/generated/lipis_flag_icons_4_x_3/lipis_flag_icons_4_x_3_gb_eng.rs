use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gb_eng (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-gb-eng" viewBox = "0 0 640 480" > < path fill = "#fff" d = "M0 0h640v480H0z" /> { "
  " } < path fill = "#ce1124" d = "M281.6 0h76.8v480h-76.8z" /> { "
  " } < path fill = "#ce1124" d = "M0 201.6h640v76.8H0z" /> </ svg > } }