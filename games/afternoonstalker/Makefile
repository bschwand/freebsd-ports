PORTNAME=	afternoonstalker
PORTVERSION=	1.2.0
CATEGORIES=	games
MASTER_SITES=	http://perso.b2b2c.ca/~sarrazip/dev/

MAINTAINER=	martymac@FreeBSD.org
COMMENT=	Clone of the 1981 Night Stalker video game
WWW=		http://sarrazip.com/dev/afternoonstalker.html

LICENSE=	GPLv2+
LICENSE_FILE=	${WRKSRC}/COPYING

LIB_DEPENDS=	libflatzebra-0.2.so:devel/flatzebra

USES=		pkgconfig sdl
USE_SDL=	sdl2 gfx2 image2 mixer2 ttf2

GNU_CONFIGURE=	yes

PORTDOCS=	AUTHORS NEWS README THANKS

OPTIONS_DEFINE=	DOCS

post-patch:
	@${REINPLACE_CMD} 's|^pkgsounddir =.*|pkgsounddir = ${DATADIR}/sounds|' \
		${WRKSRC}/src/Makefile.in
	@${REINPLACE_CMD} 's|^docdir = .*|docdir = ${DOCSDIR}|' \
		${WRKSRC}/Makefile.in

.include <bsd.port.mk>
