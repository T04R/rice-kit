#!/bin/bash
# Script to auto-detect NTFS, ext4, and Btrfs partitions and add them to /etc/fstab

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root."
    exit 1
fi

BASE_MOUNT_DIR="/mnt"
FSTAB_FILE="/etc/fstab"
TMP_FSTAB="/tmp/fstab_tmp"
declare -A FS_OPTIONS=(
    [ntfs]="defaults,uid=1000,gid=1000,dmask=0002,fmask=0113"
    [ext4]="defaults,noatime"
    [btrfs]="defaults,noatime,compress=zstd"
)

sanitize_dir() {
    echo "$1" | sed 's/[^a-zA-Z0-9_.-]/-/g'
}

cp "$FSTAB_FILE" "${FSTAB_FILE}.bak"

added_count=0
while read -r dev; do
    fstype=$(lsblk -no FSTYPE "$dev")

    [[ -z $fstype || ! "${!FS_OPTIONS[@]}" =~ $fstype ]] && continue

    uuid=$(lsblk -no UUID "$dev")
    label=$(lsblk -no LABEL "$dev")
    [[ -z $label ]] && label=$(basename "$dev")

    dir_name=$(sanitize_dir "$label")
    mount_point="${BASE_MOUNT_DIR}/${dir_name}"

    if [[ ! -d "$mount_point" ]]; then
        mkdir -p "$mount_point"
    fi

    if grep -q "UUID=$uuid" "$FSTAB_FILE"; then
        echo "⚠️  Partition $label (UUID=$uuid) already exists in fstab. Skipping."
        continue
    fi

    options="${FS_OPTIONS[$fstype]}"
    entry="UUID=$uuid $mount_point $fstype $options 0 0"
    echo "$entry" >> "$TMP_FSTAB"
    echo "✅ Added: $label (UUID=$uuid) to $mount_point"
    ((added_count++))
done < <(lsblk -no PATH -r | grep -v loop)

if [[ $added_count -gt 0 ]]; then
    cat "$TMP_FSTAB" >> "$FSTAB_FILE"
    echo -e "\n$added_count new partitions added to fstab."
    rm "$TMP_FSTAB"

    echo -e "\nAttempting to auto-mount partitions..."
    if mount -a; then
        echo "All partitions mounted successfully!"
    else
        echo "Error: Some partitions failed to mount. Please check your fstab."
    fi
else
    echo "No new partitions found."
    rm "$TMP_FSTAB"
fi



