-- Add migration script here
-- ==============================================
-- 🧱 MIGRATION: ADD USER ROLE (user_role ENUM)
-- ==============================================

-- 1) ตรวจว่ามี type 'user_role' อยู่รึยัง ถ้ายังไม่มีให้สร้างใหม่
DO $$ BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
    -- enum สำหรับบทบาทของผู้ใช้
    CREATE TYPE user_role AS ENUM ('user','admin','master');
  END IF;
END $$;

-- ----------------------------------------------
-- 2) เพิ่มคอลัมน์ role ให้ตาราง users
-- ----------------------------------------------
ALTER TABLE users
  ADD COLUMN IF NOT EXISTS role user_role      -- ใช้ type enum ที่สร้างไว้ด้านบน
  NOT NULL DEFAULT 'user';                     -- ค่า default ทุก user จะเป็น 'user'

-- ----------------------------------------------
-- 3) เพิ่ม enum สำหรับสถานะบัญชี (account_status)
-- ----------------------------------------------
DO $$ BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'account_status') THEN
    CREATE TYPE account_status AS ENUM ('pending','active','suspended','disabled');
  END IF;
END $$;

-- ----------------------------------------------
-- 4) เพิ่มคอลัมน์ status ให้ users (default = pending)
-- ----------------------------------------------
ALTER TABLE users
  ADD COLUMN IF NOT EXISTS status account_status
  NOT NULL DEFAULT 'pending';

-- ----------------------------------------------
-- 5) อัปเดต user หลัก (ของโดนัท) ให้เป็น master + active
-- 👉 เปลี่ยน email ด้านล่างให้ตรงกับของนายเอง
-- ----------------------------------------------
UPDATE users
SET role = 'master',
    status = 'active'
WHERE lower(email) = lower('donut@example.com');

-- ==============================================
-- ✅ เสร็จแล้ว: เพิ่ม role/status และตั้ง master ให้ยูสหลัก
-- ==============================================

