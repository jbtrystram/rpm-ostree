//! Helpers for filesystem metadata.
// SPDX-License-Identifier: Apache-2.0 OR MIT
use std::borrow::Cow;

// This function was copied from https://github.com/uutils/coreutils/blob/64f3cd748d5a188d5123790110d1d8ca7b33f18d/src/uucore/src/lib/features/fsext.rs#L748
pub(crate) fn pretty_fstype<'a>(fstype: i64) -> Cow<'a, str> {
    // spell-checker:disable
    match fstype {
        0x6163_6673 => "acfs".into(),
        0xADF5 => "adfs".into(),
        0xADFF => "affs".into(),
        0x5346_414F => "afs".into(),
        0x0904_1934 => "anon-inode FS".into(),
        0x6175_6673 => "aufs".into(),
        0x0187 => "autofs".into(),
        0x4246_5331 => "befs".into(),
        0x6264_6576 => "bdevfs".into(),
        0x1BAD_FACE => "bfs".into(),
        0xCAFE_4A11 => "bpf_fs".into(),
        0x4249_4E4D => "binfmt_misc".into(),
        0x9123_683E => "btrfs".into(),
        0x7372_7279 => "btrfs_test".into(),
        0x00C3_6400 => "ceph".into(),
        0x0027_E0EB => "cgroupfs".into(),
        0xFF53_4D42 => "cifs".into(),
        0x7375_7245 => "coda".into(),
        0x012F_F7B7 => "coh".into(),
        0x6265_6570 => "configfs".into(),
        0x28CD_3D45 => "cramfs".into(),
        0x453D_CD28 => "cramfs-wend".into(),
        0x6462_6720 => "debugfs".into(),
        0x1373 => "devfs".into(),
        0x1CD1 => "devpts".into(),
        0xF15F => "ecryptfs".into(),
        0xDE5E_81E4 => "efivarfs".into(),
        0x0041_4A53 => "efs".into(),
        0x5DF5 => "exofs".into(),
        0x137D => "ext".into(),
        0xEF53 => "ext2/ext3".into(),
        0xEF51 => "ext2".into(),
        0xF2F5_2010 => "f2fs".into(),
        0x4006 => "fat".into(),
        0x1983_0326 => "fhgfs".into(),
        0x6573_5546 => "fuseblk".into(),
        0x6573_5543 => "fusectl".into(),
        0x0BAD_1DEA => "futexfs".into(),
        0x0116_1970 => "gfs/gfs2".into(),
        0x4750_4653 => "gpfs".into(),
        0x4244 => "hfs".into(),
        0x482B => "hfs+".into(),
        0x4858 => "hfsx".into(),
        0x00C0_FFEE => "hostfs".into(),
        0xF995_E849 => "hpfs".into(),
        0x9584_58F6 => "hugetlbfs".into(),
        0x1130_7854 => "inodefs".into(),
        0x0131_11A8 => "ibrix".into(),
        0x2BAD_1DEA => "inotifyfs".into(),
        0x9660 => "isofs".into(),
        0x4004 => "isofs".into(),
        0x4000 => "isofs".into(),
        0x07C0 => "jffs".into(),
        0x72B6 => "jffs2".into(),
        0x3153_464A => "jfs".into(),
        0x6B41_4653 => "k-afs".into(),
        0xC97E_8168 => "logfs".into(),
        0x0BD0_0BD0 => "lustre".into(),
        0x5346_314D => "m1fs".into(),
        0x137F => "minix".into(),
        0x138F => "minix (30 char.)".into(),
        0x2468 => "minix v2".into(),
        0x2478 => "minix v2 (30 char.)".into(),
        0x4D5A => "minix3".into(),
        0x1980_0202 => "mqueue".into(),
        0x4D44 => "msdos".into(),
        0x564C => "novell".into(),
        0x6969 => "nfs".into(),
        0x6E66_7364 => "nfsd".into(),
        0x3434 => "nilfs".into(),
        0x6E73_6673 => "nsfs".into(),
        0x5346_544E => "ntfs".into(),
        0x9FA1 => "openprom".into(),
        0x7461_636F => "ocfs2".into(),
        0x794C_7630 => "overlayfs".into(),
        0xAAD7_AAEA => "panfs".into(),
        0x5049_5045 => "pipefs".into(),
        0x7C7C_6673 => "prl_fs".into(),
        0x9FA0 => "proc".into(),
        0x6165_676C => "pstorefs".into(),
        0x002F => "qnx4".into(),
        0x6819_1122 => "qnx6".into(),
        0x8584_58F6 => "ramfs".into(),
        0x5265_4973 => "reiserfs".into(),
        0x7275 => "romfs".into(),
        0x6759_6969 => "rpc_pipefs".into(),
        0x7363_6673 => "securityfs".into(),
        0xF97C_FF8C => "selinux".into(),
        0x4341_5D53 => "smackfs".into(),
        0x517B => "smb".into(),
        0xFE53_4D42 => "smb2".into(),
        0xBEEF_DEAD => "snfs".into(),
        0x534F_434B => "sockfs".into(),
        0x7371_7368 => "squashfs".into(),
        0x6265_6572 => "sysfs".into(),
        0x012F_F7B6 => "sysv2".into(),
        0x012F_F7B5 => "sysv4".into(),
        0x0102_1994 => "tmpfs".into(),
        0x7472_6163 => "tracefs".into(),
        0x2405_1905 => "ubifs".into(),
        0x1501_3346 => "udf".into(),
        0x0001_1954 => "ufs".into(),
        0x5419_0100 => "ufs".into(),
        0x9FA2 => "usbdevfs".into(),
        0x0102_1997 => "v9fs".into(),
        0xBACB_ACBC => "vmhgfs".into(),
        0xA501_FCF5 => "vxfs".into(),
        0x565A_4653 => "vzfs".into(),
        0x5346_4846 => "wslfs".into(),
        0xABBA_1974 => "xenfs".into(),
        0x012F_F7B4 => "xenix".into(),
        0x5846_5342 => "xfs".into(),
        0x012F_D16D => "xia".into(),
        0x2FC1_2FC1 => "zfs".into(),
        other => format!("UNKNOWN ({:#x})", other).into(),
    }
    // spell-checker:enable
}