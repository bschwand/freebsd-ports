PORTNAME=	ammonite
DISTVERSION=	3.0.0-M1
CATEGORIES=	devel java
MASTER_SITES=	https://github.com/lihaoyi/Ammonite/releases/download/${DISTVERSION}/
DISTNAME=	3.3-${DISTVERSION}
EXTRACT_SUFX=
DIST_SUBDIR=	${PORTNAME}
EXTRACT_ONLY=

MAINTAINER=	freebsd-ports@jan0sch.de
COMMENT=	Provides a system shell in the high-level Scala language
WWW=		https://ammonite.io/

LICENSE=	MIT

USE_JAVA=	yes
JAVA_VERSION=	8+

NO_ARCH=	yes
NO_BUILD=	yes

PLIST_FILES=	bin/ammonite

do-extract:
	@${MKDIR} ${WRKSRC}
	@${CP} ${DISTDIR}/${DIST_SUBDIR}/${DISTNAME} ${WRKSRC}

do-install:
	${INSTALL_SCRIPT} ${WRKSRC}/${DISTNAME} \
		${STAGEDIR}${PREFIX}/bin/${PORTNAME}

.include <bsd.port.mk>
