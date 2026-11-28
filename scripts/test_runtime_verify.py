"""Focused regression tests for runtime verification of portable tar archives."""

import gzip
import io
import tarfile
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import scripts.runtime_verify as runtime_verify


class VerifyTarGzTests(unittest.TestCase):
    def write_archive(self, path: Path, members: list[tuple[tarfile.TarInfo, bytes | None]]) -> None:
        with path.open("wb") as raw:
            with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as compressed:
                with tarfile.open(fileobj=compressed, mode="w") as archive:
                    for member, content in members:
                        member.mtime = 0
                        if content is None:
                            archive.addfile(member)
                        else:
                            member.size = len(content)
                            archive.addfile(member, io.BytesIO(content))

    @staticmethod
    def file_member(name: str, content: bytes) -> tuple[tarfile.TarInfo, bytes]:
        member = tarfile.TarInfo(name)
        return member, content

    def run_with_legacy_default(self, archive: Path, root: Path) -> runtime_verify.CheckResult:
        with patch.object(
            tarfile.TarFile,
            "extraction_filter",
            staticmethod(tarfile.fully_trusted_filter),
            create=True,
        ), patch.object(tempfile, "tempdir", str(root)):
            return runtime_verify.verify_tar_gz(archive, "linux", root)

    def assert_rejected_without_escape(self, member: tarfile.TarInfo, content: bytes | None = None) -> None:
        with tempfile.TemporaryDirectory(prefix="liora-tar-test-") as temporary:
            root = Path(temporary)
            archive = root / "unsafe.tar.gz"
            marker = root / "outside-marker"
            marker.write_bytes(b"trusted")
            self.write_archive(
                archive,
                [self.file_member("bin/liora-gallery", b"binary"), self.file_member("README.md", b"docs"), (member, content)],
            )
            smoke = runtime_verify.CheckResult("linux", "liora-gallery", "executable", "run", "pass", "ok", 0.0)

            with patch.object(runtime_verify, "smoke_executable", return_value=smoke):
                result = self.run_with_legacy_default(archive, root)

            self.assertEqual(result.status, "fail")
            self.assertEqual(marker.read_bytes(), b"trusted")

    def test_traversal_member_is_rejected_without_writing_outside_extraction(self):
        member, content = self.file_member("../outside-marker", b"escaped")
        self.assert_rejected_without_escape(member, content)

    def test_unsafe_symlink_is_rejected(self):
        member = tarfile.TarInfo("bin/escape")
        member.type = tarfile.SYMTYPE
        member.linkname = "../../outside-marker"
        self.assert_rejected_without_escape(member)

    def test_special_file_is_rejected(self):
        member = tarfile.TarInfo("bin/device")
        member.type = tarfile.FIFOTYPE
        self.assert_rejected_without_escape(member)

    def test_valid_archive_still_extracts_and_smokes_binary(self):
        with tempfile.TemporaryDirectory(prefix="liora-tar-test-") as temporary:
            root = Path(temporary)
            archive = root / "valid.tar.gz"
            self.write_archive(archive, [self.file_member("bin/liora-gallery", b"binary"), self.file_member("README.md", b"docs")])
            smoke = runtime_verify.CheckResult("linux", "liora-gallery", "executable", "run", "pass", "ok", 0.0)

            with patch.object(runtime_verify, "smoke_executable", return_value=smoke) as smoke_executable:
                result = self.run_with_legacy_default(archive, root)

            self.assertEqual(result.status, "pass")
            smoke_executable.assert_called_once()



if __name__ == "__main__":
    unittest.main()
