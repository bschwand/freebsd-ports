PORTNAME=	line-protocol-parser
PORTVERSION=	1.1.1
PORTREVISION=	1
CATEGORIES=	textproc python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Parses InfluxDB line protocol strings into Python dictionaries
WWW=		https://github.com/Penlect/line-protocol-parser/

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE.md

USES=		python
USE_GITHUB=	yes
USE_PYTHON=	autoplist concurrent distutils

GH_ACCOUNT=	Penlect
GH_PROJECT=	line-protocol-parser

do-test:
	@cd ${WRKSRC} && ${PYTHON_CMD} ${PYDISTUTILS_SETUP} test

.include <bsd.port.mk>
