PORTNAME=	lua-resty-lrucache
DISTVERSIONPREFIX=	v
DISTVERSION=	0.15
CATEGORIES=	www

MAINTAINER=	zi@FreeBSD.org
COMMENT=	Lua-land LRU cache based on the LuaJIT FFI
WWW=		https://github.com/openresty/lua-resty-lrucache

LICENSE=	BSD2CLAUSE

RUN_DEPENDS=	luajit-openresty>=2.1:lang/luajit-openresty

USE_GITHUB=	yes
GH_ACCOUNT=	openresty

NO_ARCH=	yes
NO_BUILD=	yes

LUA_MODSHAREDIR=	${LOCALBASE}/share/lua/5.1

PLIST_FILES+=	${LUA_MODSHAREDIR}/resty/lrucache.lua \
		${LUA_MODSHAREDIR}/resty/lrucache/pureffi.lua

do-install:
	@${MKDIR} ${STAGEDIR}${LUA_MODSHAREDIR}
	@(cd ${WRKSRC}/lib && ${COPYTREE_SHARE} . ${STAGEDIR}${LUA_MODSHAREDIR} "-name *\.lua")

.include <bsd.port.mk>
