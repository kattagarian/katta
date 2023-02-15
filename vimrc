" vimrc
set ruler		" Show current position
set hlsearch		" Highlight search
set number		" Show line number
set showmatch		" Show matching brackets 
set title		" Show file in title bar
set relativenumber	
syntax enable 

" highlight current line
au WinLeave * set nocursorline nocursorcolumn
au WinEnter * set cursorline cursorcolumn
set cursorline cursorcolumn
filetype plugin indent on

set background=dark

