PORTNAME=	prometheus-zfs
DISTVERSIONPREFIX=	v
DISTVERSION=	1.0.4
CATEGORIES=	filesystems sysutils python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	0mp@FreeBSD.org
COMMENT=	Prometheus exporter for ZFS statistics using py-libzfs
WWW=		https://github.com/matusnovak/prometheus-zfs

LICENSE=	UNLICENSE
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}libzfs>=0:filesystems/py-libzfs@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}prometheus-client>=0:net-mgmt/py-prometheus-client@${PY_FLAVOR}

USES=		python shebangfix
USE_GITHUB=	yes
GH_ACCOUNT=	matusnovak
USE_RC_SUBR=	zfsprom
SHEBANG_FILES=	zfsprom.py

NO_ARCH=	yes
NO_BUILD=	yes

SUB_LIST=	PYTHON_CMD=${PYTHON_CMD}

PLIST_FILES=	sbin/zfsprom.py

do-install:
	${INSTALL_SCRIPT} ${WRKSRC}/zfsprom.py ${STAGEDIR}${PREFIX}/sbin

.include <bsd.port.mk>
