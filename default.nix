{ lib, stdenv
  ,naersk
	,rustc
	,cargo
	,pkg-config
}:

naersk.buildPackage rec {
  pname = "standard";
  version = "0.1.0";

  src = ./.;

  meta = with lib; {
    homepage = "HOMEPAGE";
    description = ''
      DESCRIPTION
    '';
    licencse = licenses.mit;
    platforms = platforms.all;
    maintainers = with maintainers; [ xdHampus ];
  };
}
