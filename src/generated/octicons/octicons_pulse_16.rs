use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn octicons_pulse_16 (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 16 16" >< path fill - rule = "evenodd" d = "M6 2a.75.75 0 01.696.471L10 10.731l1.304-3.26A.75.75 0 0112 7h3.25a.75.75 0 010 1.5h-2.742l-1.812 4.528a.75.75 0 01-1.392 0L6 4.77 4.696 8.03A.75.75 0 014 8.5H.75a.75.75 0 010-1.5h2.742l1.812-4.529A.75.75 0 016 2z" /></ svg > } }