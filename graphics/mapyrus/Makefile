PORTNAME=	mapyrus
PORTVERSION=	1.802
PORTREVISION=	0
CATEGORIES=	graphics print java
MASTER_SITES=	SF

MAINTAINER=	nivit@FreeBSD.org
COMMENT=	Software for creating plots of points, lines, etc
WWW=		https://mapyrus.sourceforge.net

LICENSE=	LGPL21
LICENSE_FILE=	${WRKSRC}/userdoc/COPYING

DATADIR=	${JAVASHAREDIR}/${PORTNAME}

MAPYRUSDOCS=	${PORTNAME}.pdf
MAPYRUSJAR=	${PORTNAME}.jar

NO_BUILD=	yes
NO_WRKSUBDIR=	yes

OPTIONS_DEFINE=	DOCS EXAMPLES

SUB_FILES=	${PORTNAME} pkg-message
SUB_LIST=	MAPYRUSJAR=${MAPYRUSJAR}

USES=		java zip

do-install:
	@${MKDIR} ${STAGEDIR}${DATADIR}
	${INSTALL_DATA} ${WRKSRC}/${MAPYRUSJAR} ${STAGEDIR}${DATADIR}/${MAPYRUSJAR}
	${INSTALL_SCRIPT} ${WRKDIR}/${PORTNAME} ${STAGEDIR}${PREFIX}/bin/${PORTNAME}
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${WRKSRC}/${MAPYRUSDOCS} ${STAGEDIR}${DOCSDIR}/${MAPYRUSDOCS}
	@${MKDIR} ${STAGEDIR}${EXAMPLESDIR}
	cd ${WRKSRC}/userdoc && \
	${COPYTREE_SHARE} . ${STAGEDIR}${EXAMPLESDIR} \
		"-type f -not -iname copying -and -not -iname *\.tex"

.include <bsd.port.mk>
