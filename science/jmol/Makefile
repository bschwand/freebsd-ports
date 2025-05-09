PORTNAME=	Jmol
DISTVERSION=	16.1.25
PORTREVISION=	1
CATEGORIES=	science java
MASTER_SITES=	SF/${PORTNAME:tl}/${PORTNAME}/Version%20${DISTVERSION:R}/${PORTNAME}%20${DISTVERSION}/
DISTFILES=	${PORTNAME}-${DISTVERSION}-full${EXTRACT_SUFX}

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	Java 3D viewer for chemical structures
WWW=		http://jsmol.sourceforge.net/

LICENSE=	LGPL21
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

USES=		java:ant,run
JAVA_VERSION=	17 # Java 8 fails on aarch64: Internal Error (assembler_aarch64.hpp:237), pid=91175, tid=0x000000000007cda5 guarantee(val < (1U << nbits)) failed: Field too big for insn

NO_ARCH=	yes

WRKSRC=		${WRKDIR}/${PORTNAME:tl}-${DISTVERSION}
SIGN_DIR=	${WRKSRC}/selfSignedCertificate
SUB_FILES=	${EXES}
SUB_LIST=	JAVA=${JAVA}

EXES=		${PORTNAME:tl} ${PORTNAME:tl}data
JARS=		${PORTNAME}.jar Jvxl.jar ${PORTNAME}Data.jar # ${PORTNAME}Lib.jar

PLIST_FILES=	${EXES:S/^/bin\//} \
		${JARS:S/^/${JAVAJARDIR}\//}

OPTIONS_DEFINE=		DOCS JSMOL
OPTIONS_DEFAULT=	JSMOL

JSMOL_DESC=		Install JSmol
JSMOL_BUILD_DEPENDS=	unzip>0:archivers/unzip
JSMOL_VARS=		WWWDIR=${PREFIX}/www/JSmol PLIST=${.CURDIR}/pkg-plist-jsmol

PORTDOCS=	*

DESKTOP_ENTRIES="${PORTNAME}" "" "" "${PORTNAME:tl}" "" false

post-patch:
	@${REINPLACE_CMD} 's|<input message="private property file.*||' ${WRKSRC}/build.xml

post-build-JSMOL-on:
	@${MKDIR} ${WRKDIR}${PREFIX}/www
	@${UNZIP_CMD} -d ${WRKDIR}${PREFIX}/www ${WRKSRC}/jsmol.zip

do-install:
.for exe in ${EXES}
	${INSTALL_SCRIPT} ${WRKDIR}/${exe} ${STAGEDIR}${PREFIX}/bin
.endfor
.for jar in ${JARS}
	${INSTALL_DATA} ${WRKSRC}/build/${jar} ${STAGEDIR}${JAVAJARDIR}
.endfor

do-install-JSMOL-on:
	${MKDIR} ${STAGEDIR}${WWWDIR}
	${CP} -r ${WRKDIR}${PREFIX}/www/jsmol/* ${STAGEDIR}${WWWDIR}
	${RM} ${STAGEDIR}${WWWDIR}/debug.log

do-install-DOCS-on:
	cd ${WRKSRC}/doc && ${COPYTREE_SHARE} . ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
