PORTNAME=	opentelemetry-instrumentation-active_model_serializers
PORTVERSION=	0.21.0
PORTREVISION=	1
CATEGORIES=	devel rubygems
MASTER_SITES=	RG

MAINTAINER=	sunpoet@FreeBSD.org
COMMENT=	Active Model Serializers instrumentation for the OpenTelemetry framework
WWW=		https://opentelemetry.io/ \
		https://github.com/open-telemetry/opentelemetry-ruby-contrib \
		https://github.com/open-telemetry/opentelemetry-ruby-contrib/tree/main/instrumentation/active_model_serializers

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	rubygem-opentelemetry-api>=1.0<2:devel/rubygem-opentelemetry-api \
		rubygem-opentelemetry-instrumentation-active_support>=0.7.0:devel/rubygem-opentelemetry-instrumentation-active_support \
		rubygem-opentelemetry-instrumentation-base>=0.22.1<0.23:devel/rubygem-opentelemetry-instrumentation-base

USES=		gem

NO_ARCH=	yes

.include <bsd.port.mk>
