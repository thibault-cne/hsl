# This is a makefile to build multiples executable.

# Add C files here
FILES_1	=	test.s

FILES_2	=	printf.s

# Note that C files must be in
# the srcs folder
# header files must be in the includes
# folder, object files will be build 
# inside the objs folder the binary
# will be created inside the build folder.
S_PATH	=	srcs/
O_PATH	=	objs/
I_PATH	=	includes/
B_PATH	=	build/

CC		=	as
LINKER	=	ld

# Name : the binary will have this name
NAME_1	=	test
NAME_2	=	printf

# Need to create SRCS_* AND OBJS_* for each executable
SRCS_1 	=	$(addprefix ${S_PATH}, ${FILES_1})
OBJS_1 	=	$(addprefix ${O_PATH}, ${FILES_1:.s=.o})

SRCS_2 	=	$(addprefix ${S_PATH}, ${FILES_2})
OBJS_2 	=	$(addprefix ${O_PATH}, ${FILES_2:.s=.o})

# Numbers refers to the number of jobs
NUMBERS	=	$(shell seq 1 2)
JOBS	=	$(addprefix job, ${NUMBERS})

RM		=	rm -rf

# Change flags regarding the OS
CCFLAGS		=	
LDFLAGS		=	-lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64

${O_PATH}%.o:	${S_PATH}%.s
				@mkdir -p ${dir $@}
				@${CC} ${CCFLAGS} -o $@ $<
				@echo ${ECHO_FLAG} "${ESC}${BOLD}${ESC}${CYAN}Compiling${ESC}${S}${ESC}${S} ${ESC}${IGREY}$<${ESC}${S} ${ESC}${YELLOW}-> ${ESC}${S}${ESC}${SBLUE}$@${ESC}${S} ⚙️"

.SECONDEXPANSION:
${JOBS}: job%:	$${OBJS_$$*} build;
				@${LINKER} -macos_version_min 14.0.0 -o ${B_PATH}${NAME_$*} ${OBJS_$*} ${LDFLAGS}
				@echo ""
				@echo ${ECHO_FLAG} "${ESC}${BOLD}${ESC}${PURPLE}Building${ESC}${S}${ESC}${S} ${ESC}${IGREY}$@${ESC}${S} 🖥️"
				@echo ""
				@echo ${ECHO_FLAG} "${ESC}${ITALIC}${ESC}${SGREEN}Compilation is completed !${ESC}${S} 🎉"

all:			${JOBS}

clean:
				@${RM} ${O_PATH}*
				@echo ${ECHO_FLAG} "${ESC}${BOLD}${ESC}${SRED}Removing${ESC}${S}${ESC}${S} ${ESC}${IGREY}${O_PATH}${ESC}${S} 🗑️"

fclean:			clean
				@${RM} ${B_PATH}*
				@echo ${ECHO_FLAG} "${ESC}${BOLD}${ESC}${SRED}Removing${ESC}${S}${ESC}${S} ${ESC}${IGREY}${B_PATH}${ESC}${S} 🗑️"

space:
			@echo " "

re:			fclean space all

build:
			@mkdir -p ${B_PATH}

.PHONY: all clean fclean re build space

ESC		=		\033
S 		=		[0m
BOLD 	= 		[1m
ITALIC 	= 		[3m
UNDER 	= 		[4m
REV 	= 		[7m

# Colors
GREY 	= 		[30m
RED 	= 		[31m
GREEN	=		[32m
YELLOW	=		[33m
BLUE	=		[34m
PURPLE	=		[35m
CYAN	=		[36m
WHITE	=		[37m

SGREY	=		[90m
SRED	=		[91m
SGREEN	=		[92m
SYELLOW	=		[93m
SBLUE	=		[94m
SPURPLE	=		[95m
SCYAN	=		[96m
SWHITE	=		[97m

# Colored backgrounds
IGREY	=		[40m
IRED	=		[41m
IGREEN	=		[42m
IYELLOW	=		[43m
IBLUE	=		[44m
IPURPLE	=		[45m
ICYAN	=		[46m
IWHITE	=		[47m
