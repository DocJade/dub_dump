//x: 80, y: 30
//    00000000001111111111222222222233333333334444444444555555555566666666667777777777
//    01234567890123456789012345678901234567890123456789012345678901234567890123456789

//00         .____        _      .____                          |     Statistics:     
//01         |    \  _ _ | [_    |    \  _ _  _____ .___        | Total clips:  12345 
//02         |  |  || | || . |   |  |  || | || , , || . |       | Dumped clips: 12345 
//03         |____/ |___||___|   |____/ |___||_|_|_||  _|       | Cut ratio:    .0985  // logo is horizontally centered in its box
//04                                                |_|         | Old run time: 0.00H 
//05   -----space for version text----------------------------- | New run time: 0.00H 
//06   -----MOTD---------------56-chars-of-space--------------- | Time saved:   0.00H 
//07  ----------------------------------------------------------+--------------------- // divider
//08   --total-directory-progress-bar--------------------------------------------000%  // style is 1234 ##|'.'.'. 1234 //.'.' might change
//09  -------------------------------------------------------------------------------- // divider
//11  ################################################################################
//12  ################################################################################ // extra space? what goes here?
//13  ################################################################################ // i could also reduce the app hight
//14  ################################################################################
//15                                                                                   // gap
//16             .                     .:####.                                         // audio clip waveform
//17          .:|#.                 .:|#######.                                        // audio clip waveform
//18      .:|######:            .:|############:                                       // audio clip waveform  .:|#
//19  -:|###########:--------:|#################:------------------------------------- // audio clip waveform  -:|#
//20      ':|######:            ':|############:                                       // audio clip waveform  ':|#
//21          ':|#'                 ':|#######'                                        // audio clip waveform
//22             '                     :|####'                                         // audio clip waveform
//23                                                                                   // gap
//24   --clip-progress-bar-----------------------------------------------------------  // style is 0m00s -|----- 0m00s
//23                                                                                   // gap
//25  -------------------------------------------------------------------------------- // divider
//26  ################################################################################ // timeline view, shows relative clip lengths to eachother, current clip in middle
//27  -------------------------------------------------------------------------------- // divider
//28  space: replay     s: skip     d: dump     x: reset speed     ^c: exit            // volume / speed?
//29  up/down: volume     left/right: speed                                  [DocJade] // keybinds

// keybinds

// space: again (play sound again from beginning)
// s: skip / save
// d: dump / discard (deletes the copy)
// up: vol up
// down: vol down
// right: speed up
// left: slow down
// x: reset speed.