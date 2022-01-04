set pcpoA 20
set pcpoB 5

menubutton $Global(user_menu).alm \
    -menu $Global(user_menu).alm.m \
    -text {ALM} \
    -underline {0}
menu $Global(user_menu).alm.m
pack $Global(user_menu).alm -in $Global(user_menu) -side left

$Global(user_menu).alm.m add command \
    -command pcpo_init \
    -label "Initialiser a=$pcpoA,b=$pcpoB" \
    -underline {0}

proc pcpo_init {} {
    global Global InputVars

    set InputVars(chAExt) 1
    set InputVars(chRExt) 1
    set InputVars(input_2) 1
    set InputVars(input_4) 1
    TreatStep
    set InputVars(chAExt) 0
    set InputVars(chRExt) 0
    set InputVars(chBExt) 1
    set InputVars(input_4) 0
    set InputVars(input_0) 1
    TreatStep
    set InputVars(chBExt) 0
    set InputVars(input_0) 0
    set InputVars(input_2) 0
    TreatStep
    set InputVars(On) 1
}
