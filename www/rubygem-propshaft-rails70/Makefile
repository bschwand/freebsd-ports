PORTNAME=	propshaft
PORTVERSION=	1.1.0
CATEGORIES=	www rubygems
MASTER_SITES=	RG
PKGNAMESUFFIX=	-rails70

MAINTAINER=	sunpoet@FreeBSD.org
COMMENT=	Deliver assets for Rails
WWW=		https://github.com/rails/propshaft

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/MIT-LICENSE

RUN_DEPENDS=	rubygem-actionpack70>=7.0.0:www/rubygem-actionpack70 \
		rubygem-activesupport70>=7.0.0:devel/rubygem-activesupport70 \
		rubygem-rack>=0,3:www/rubygem-rack \
		rubygem-railties70>=7.0.0:www/rubygem-railties70

USES=		gem

NO_ARCH=	yes

.include <bsd.port.mk>
