PORTNAME=	Crypt-DSA
PORTVERSION=	1.19
CATEGORIES=	security perl5
MASTER_SITES=	CPAN
PKGNAMEPREFIX=	p5-

MAINTAINER=	perl@FreeBSD.org
COMMENT=	DSA signature and key generation
WWW=		https://metacpan.org/release/Crypt-DSA

BUILD_DEPENDS=	${RUN_DEPENDS}
RUN_DEPENDS=	p5-Digest-SHA1>=0:security/p5-Digest-SHA1 \
		p5-Convert-PEM>=0:converters/p5-Convert-PEM \
		p5-Data-Buffer>=0:misc/p5-Data-Buffer \
		p5-File-Which>=0.05:sysutils/p5-File-Which

USES=		perl5
USE_PERL5=	configure

NO_ARCH=	yes

.include <bsd.port.mk>
