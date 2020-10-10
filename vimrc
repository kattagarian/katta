" vimrc
syntax enable
set background=dark
colorscheme solarized
set ruler		" Show current position
set hlsearch		" Highlight search
set number		" Show line number
set showmatch		" Show matching brackets 
set title		" Show file in title bar
set relativenumber	
" highlight current line
au WinLeave * set nocursorline nocursorcolumn
au WinEnter * set cursorline cursorcolumn
set cursorline cursorcolumn
