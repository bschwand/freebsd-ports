PORTNAME=	pyformance
PORTVERSION=	0.4
PORTREVISION=	1
CATEGORIES=	devel python
MASTER_SITES=	PYPI
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	waitman@waitman.net
COMMENT=	Toolset for performance measurement and statistics
WWW=		https://pypi.org/project/pyformance/

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}six>=1.11.0:devel/py-six@${PY_FLAVOR}

OPTIONS_DEFINE=	DOCS

NO_ARCH=	yes
USES=		python
USE_PYTHON=	autoplist concurrent distutils

PORTDOCS=	README.md

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${WRKSRC}/README.md ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
