use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn octicons_rss_24 (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 24 24" >< path fill - rule = "evenodd" d = "M3.5 3.25a.75.75 0 01.75-.75C14.053 2.5 22 10.447 22 20.25a.75.75 0 01-1.5 0C20.5 11.275 13.225 4 4.25 4a.75.75 0 01-.75-.75zM3.5 19a2 2 0 114 0 2 2 0 01-4 0zm.75-9.5a.75.75 0 000 1.5 9.25 9.25 0 019.25 9.25.75.75 0 001.5 0C15 14.313 10.187 9.5 4.25 9.5z" /></ svg > } }