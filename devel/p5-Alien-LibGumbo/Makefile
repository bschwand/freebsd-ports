PORTNAME=	Alien-LibGumbo
PORTVERSION=	0.05
PORTREVISION=	2
CATEGORIES=	devel perl5
MASTER_SITES=	CPAN
PKGNAMEPREFIX=	p5-

MAINTAINER=	mikael@FreeBSD.org
COMMENT=	Gumbo parser library
WWW=		https://metacpan.org/pod/Alien::LibGumbo

LICENSE=	ART10 GPLv1+
LICENSE_COMB=	dual

LIB_DEPENDS=	libgumbo.so:devel/gumbo
BUILD_DEPENDS=	p5-Alien-Base-ModuleBuild>0:devel/p5-Alien-Base-ModuleBuild \
		p5-File-ShareDir>0:devel/p5-File-ShareDir \
		p5-Path-Class>0:devel/p5-Path-Class \
		p5-PkgConfig>0:devel/p5-PkgConfig \
		p5-URI>0:net/p5-URI
RUN_DEPENDS=	p5-Alien-Build>0:devel/p5-Alien-Build

USES=		perl5
USE_PERL5=	modbuild

EXTRACT_AFTER_ARGS=	--exclude ${DISTNAME}/gumbo-0.10.1.tar.gz \
			--no-same-owner --no-same-permissions

NO_ARCH=	yes

.include <bsd.port.mk>
