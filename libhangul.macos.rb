#!/usr/bin/env ruby
PKG_CONFIG_PATH = Dir.glob('/usr/local/Cellar/**/lib/pkgconfig').join(?:)
USE_NLS = 'no'
LTLIBINTL = '/usr/local/lib/libintl.a'
TOOL_MAKEFILE = './tools/Makefile.am'

system('brew install m4 expat gettext intltool libtool')

abort "#{LTLIBINTL} is not found" unless File.exist? LTLIBINTL

Dir.chdir(`git rev-parse --show-toplevel`.strip) do
  makefile = File.read(TOOL_MAKEFILE).gsub('$(LTLIBINTL)', LTLIBINTL)
  File.write(TOOL_MAKEFILE, "AM_LDFLAGS = -framework CoreFoundation\n#{makefile}")
  File.write('./config.rpath', '') unless File.exist? './config.rpath'

  system('./autogen.sh')
  system({'USE_NLS' => USE_NLS, 'PKG_CONFIG_PATH' => PKG_CONFIG_PATH}, './configure')
  system({'USE_NLS' => USE_NLS}, 'make')
end
