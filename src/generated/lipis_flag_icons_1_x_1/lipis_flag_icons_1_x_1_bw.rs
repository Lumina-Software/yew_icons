use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_bw (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-bw" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#00cbff" d = "M0 0h512v512H0z" /> { "
    " } < path fill = "#fff" d = "M0 192h512v128H0z" /> { "
    " } < path d = "M0 212.7h512V299H0z" /> { "
  " } </ g > </ svg > } }