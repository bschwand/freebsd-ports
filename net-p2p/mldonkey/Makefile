PORTNAME=	mldonkey
DISTVERSION=	3.1.7-2
CATEGORIES+=	net-p2p
MASTER_SITES=	https://github.com/ygrek/${PORTNAME}/releases/download/release-3-1-7-2/

MAINTAINER?=	danfe@FreeBSD.org
COMMENT?=	Multi-protocol peer-to-peer client written in OCaml
WWW=		https://mldonkey.sourceforge.net/

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/Copying.txt

CONFLICTS=	mldonkey{-{core,gui},}-devel

BUILD_DEPENDS=	${LOCALBASE}/lib/ocaml/nums.cma:math/ocaml-num

USES=		compiler:c++11-lang cpe gmake iconv ocaml:camlp4 pkgconfig tar:bzip2
ARCH!=		uname -p
.if ${ARCH} == i386 || ${ARCH} == powerpc
# `as' is used to compile src/utils/lib/md4_i386.s
USE_BINUTILS=	yes
.endif
USE_CXXSTD=	c++14

GNU_CONFIGURE=	yes
CONFIGURE_ARGS=	ac_cv_lib_charset_locale_charset=no
ALL_TARGET=	opt
MAKE_ENV+=	OCAMLRUNPARAM="l=256M"
MAKE_JOBS_UNSAFE=	yes

SUB_FILES=	pkg-message

###
## Options activation
###

OPTIONS_DEFINE=	GD DOCS EXAMPLES
OPTIONS_DEFAULT=CORE GD GUI
OPTIONS_MULTI=	CLIENT
OPTIONS_MULTI_CLIENT=CORE GUI

CORE_DESC=	Client 'core' support

.include <bsd.port.options.mk>

.if ${PORT_OPTIONS:MGUI} && ${PORT_OPTIONS:MCORE}
CONFLICTS+=	mldonkey-core-[0.9]*
CONFLICTS+=	mldonkey-gui-[0.9]*

PLIST_SUB+=	CORE="" GUI="" GUICORE=""
.else # (${PORT_OPTIONS:MGUI} && ${PORT_OPTIONS:MCORE})

. if !${PORT_OPTIONS:MGUI}
PKGNAMESUFFIX=	-core

CONFLICTS+=	mldonkey-gui-[0.9]*

CONFIGURE_ARGS+=--disable-gui
PLIST_SUB+=	CORE="" GUI="@comment " GUICORE="@comment "
. endif # !${PORT_OPTIONS:MGUI}

. if !${PORT_OPTIONS:MCORE}
PKGNAMESUFFIX=	-gui

CONFLICTS+=	mldonkey-core-[0.9]*

PLIST_SUB+=	CORE="@comment " GUI="" GUICORE="@comment "
. endif # !${PORT_OPTIONS:MCORE}

.endif # ${PORT_OPTIONS:MGUI} && ${PORT_OPTIONS:MCORE}

.if ${PORT_OPTIONS:MGUI}
BUILD_DEPENDS+=	${LOCALBASE}/lib/ocaml/site-lib/lablgtk2/lablrsvg.cma:x11-toolkits/ocaml-lablgtk2

CONFIGURE_ARGS+=--enable-gui=newgui2
# we don't need lablgtk as RUN dependency, but we need gtk+glib
USES+=		gnome
USE_GNOME=	gtk20 librsvg2
.endif # ${PORT_OPTIONS:MGUI}

.if ${PORT_OPTIONS:MCORE}
USE_RC_SUBR=	mlnet
SUB_LIST+=	SH=${SH:Q}

. if ${PORT_OPTIONS:MGD}
LIB_DEPENDS+=	libgd.so:graphics/gd
. else
CONFIGURE_ARGS+=--disable-gd
. endif
.endif

PORTDOCS=	Authors.txt Bugs.txt ChangeLog Developers.txt \
		Install.txt Todo.txt ed2k_links.txt

# build additional tools
ALL_TARGET+=	\
		mld_hash

post-patch:
	@${SED} \
		-e "s|%%PREFIX%%|${PREFIX}|" \
		-e "s|%%SH%%|${SH}|" \
		${FILESDIR}/wrapper.sh > \
		${WRKDIR}/wrapper.sh
	@${REINPLACE_CMD} \
		-e 's|$$OCAMLLIB/$$LABLGTK_NAME|$$OCAMLLIB/site-lib/$$LABLGTK_NAME|' \
		-e 's|^DEVFLAGS=|&-unsafe-string|' \
		${WRKSRC}/config/configure
	@${REINPLACE_CMD} \
		-e 's|+labl|+site-lib/labl|' \
		-e '/BITSTRING/s|OCAMLC)|& -unsafe-string|' \
		${WRKSRC}/config/Makefile.in
	@${REINPLACE_CMD} -E \
		-e 's@(#include <)(lablgtk2/)@\1site-lib/\2@'  \
		${WRKSRC}/src/gtk2/gui/x11/systraystubs.c
# update server.met provider
	@${REINPLACE_CMD} -E \
		-e 's|http://www.gruk.org/server.met.gz|http://www.jd2k.com/server.met|' \
		-e 's|http://www.bluetack.co.uk/config/antip2p.txt|http://www.bluetack.co.uk/config/level1.gz|' \
		${WRKSRC}/src/daemon/common/commonOptions.ml \
		${WRKSRC}/src/networks/donkey/donkeyServers.ml

do-install:
.if ${PORT_OPTIONS:MCORE}
	${INSTALL_PROGRAM} ${WRKSRC}/mld_hash ${STAGEDIR}${PREFIX}/bin
	${INSTALL_PROGRAM} ${WRKSRC}/mlnet ${STAGEDIR}${PREFIX}/bin/mlnet-real
	${INSTALL_SCRIPT} ${WRKDIR}/wrapper.sh ${STAGEDIR}${PREFIX}/bin/mlnet
.if ${PORT_OPTIONS:MEXAMPLES}
	@${MKDIR} ${STAGEDIR}${EXAMPLESDIR}
	${INSTALL_SCRIPT} ${FILESDIR}/kill_mldonkey ${STAGEDIR}${EXAMPLESDIR}
.endif
.endif
.if ${PORT_OPTIONS:MGUI}
	${INSTALL_PROGRAM} ${WRKSRC}/mlgui ${STAGEDIR}${PREFIX}/bin
.endif
.if ${PORT_OPTIONS:MGUI} && ${PORT_OPTIONS:MCORE}
	${INSTALL_SCRIPT} ${WRKSRC}/distrib/mldonkey_previewer ${STAGEDIR}${PREFIX}/bin
	${INSTALL_PROGRAM} ${WRKSRC}/mlguistarter ${STAGEDIR}${PREFIX}/bin
	${INSTALL_PROGRAM} ${WRKSRC}/mlnet+gui ${STAGEDIR}${PREFIX}/bin
.endif
.if ${PORT_OPTIONS:MDOCS}
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
.for _file in ${PORTDOCS}
	${INSTALL_DATA} ${WRKSRC}/distrib/${_file} ${STAGEDIR}${DOCSDIR}
.endfor
.endif

install-user: extract
.if !defined(LANGUAGE)
	@${ECHO_MSG} "Please call as:  make install-user LANGUAGE=<lang>"
	@${ECHO_MSG} "with <lang> one of en,de,fr,fr.noaccents,sp !"
.else
	@${CP} ${WRKSRC}/distrib/i18n/gui_messages.ini.${LANGUAGE}\
		${HOME}/.mldonkey_gui_messages.ini
.endif

.include <bsd.port.mk>
